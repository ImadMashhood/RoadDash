#!/usr/bin/env python3
"""Independent local UI reference reader. Retail pixels/text never enter the repo.
Exports every IMAGES file, PE bitmap/icon/cursor and optional video thumbnail.
Repository handoff contains sanitized metadata and original wireframes only.
"""
import argparse
from collections import Counter
import csv
import hashlib
import io
import json
from pathlib import Path
import struct
import subprocess

ROOT = Path(__file__).resolve().parents[1]
REFERENCE_COMMIT = '7aecf6044f2dd773b925b325823f30d648d61c89'

class PE:
    def __init__(self, data):
        self.data = data
        if data[:2] != b'MZ': raise ValueError('not a PE image')
        off = self.u32(60)
        if self.take(off, 4) != b'PE\0\0': raise ValueError('invalid PE signature')
        count, opt = self.u16(off+6), off+24
        if not 1 <= count <= 96 or self.u16(opt) != 0x10b: raise ValueError('expected bounded PE32')
        self.image_base = self.u32(opt+28)
        self.resource_rva = self.u32(opt+112)
        sections = opt+self.u16(off+20)
        self.sections = [(self.u32(sections+i*40+12), self.u32(sections+i*40+16), self.u32(sections+i*40+20)) for i in range(count)]
    def take(self, p, n):
        if p < 0 or n < 0 or p+n > len(self.data): raise ValueError('range exceeds PE data')
        return self.data[p:p+n]
    def u16(self, p): return struct.unpack('<H', self.take(p, 2))[0]
    def u32(self, p): return struct.unpack('<I', self.take(p, 4))[0]
    def rva(self, address, size):
        for va, length, offset in self.sections:
            if va <= address and address+size <= va+length:
                return self.take(offset+address-va, size)
        raise ValueError('unmapped PE address')
    def va(self, address, size): return self.rva(address-self.image_base, size)
    def resources(self):
        if not self.resource_rva: return []
        result=[]
        def read(offset, n): return self.rva(self.resource_rva+offset,n)
        def visit(offset, path, ancestors):
            if offset in ancestors or len(path)>3: raise ValueError('resource cycle/depth')
            head=read(offset,16); n=sum(struct.unpack_from('<HH',head,12))
            if n>4096: raise ValueError('resource count outside limit')
            for i in range(n):
                name, child=struct.unpack('<II',read(offset+16+i*8,8))
                if name & 0x80000000:
                    loc=name & 0x7fffffff; length=struct.unpack('<H',read(loc,2))[0]
                    if length>256: raise ValueError('resource name too long')
                    name=read(loc+2,length*2).decode('utf-16-le')
                if child & 0x80000000: visit(child & 0x7fffffff,path+[name],ancestors|{offset})
                else:
                    address,size,_,_=struct.unpack('<IIII',read(child,16))
                    if size>32*1024*1024: raise ValueError('resource exceeds limit')
                    result.append((path+[name],self.rva(address,size)))
        visit(0,[],set())
        return result
    def strings(self):
        strings={}
        for path, data in self.resources():
            if path[0]!=6 or not isinstance(path[1],int): continue
            cursor=0
            for i in range(16):
                if cursor+2>len(data): raise ValueError('truncated string block')
                n=struct.unpack_from('<H',data,cursor)[0];cursor+=2
                if cursor+n*2>len(data): raise ValueError('truncated string')
                if n: strings[(path[1]-1)*16+i]=data[cursor:cursor+n*2].decode('utf-16-le')
                cursor+=n*2
        return strings

def dialog_template(data):
    """Read standard/extended Win32 dialog templates; coordinates remain DLU."""
    cursor=0
    def read(fmt):
        nonlocal cursor
        size=struct.calcsize(fmt)
        if cursor+size>len(data):raise ValueError('truncated dialog template')
        out=struct.unpack_from(fmt,data,cursor);cursor+=size
        return out
    def atom():
        first=read('<H')[0]
        if first==0:return ''
        if first==0xffff:return {'ordinal':read('<H')[0]}
        words=[first]
        while words[-1]!=0:
            if len(words)>8192:raise ValueError('dialog text too long')
            words.append(read('<H')[0])
        return struct.pack('<'+'H'*(len(words)-1),*words[:-1]).decode('utf-16-le')
    extended=len(data)>=4 and data[:4]==b'\x01\x00\xff\xff'
    if extended:
        version,signature,help_id,exstyle,style,count,x,y,w,h=read('<HHIIIHhhhh')
    else:style,exstyle,count,x,y,w,h=read('<IIHhhhh')
    if count>256 or w<=0 or h<=0:raise ValueError('invalid dialog dimensions/count')
    menu,window_class,title=atom(),atom(),atom();font=None
    if style & 0x40:
        points=read('<H')[0]
        if extended:weight,italic,charset=read('<HBB')
        font=dict(points=points,typeface=atom())
    controls=[]
    for i in range(count):
        cursor=(cursor+3)&~3
        if extended:help_id,ex,st,cx,cy,cw,ch,ident=read('<IIIhhhhI')
        else:st,ex,cx,cy,cw,ch,ident=read('<IIhhhhH')
        cls,label=atom(),atom();extra=read('<H')[0]
        if extra:
            skip=extra if extended else extra-2
            if skip<0 or cursor+skip>len(data):raise ValueError('invalid dialog creation data')
            cursor+=skip
        controls.append(dict(index=i,control_id=ident,rect=[cx,cy,cw,ch],window_class=cls,label=label,style=st,exstyle=ex))
    return dict(title=title,rect=[x,y,w,h],units='dialog units, not pixels',format='extended' if extended else 'standard',font=font,controls=controls,bytes_consumed=cursor)

SCREEN_NAMES={0:'Reserved / bootstrap',1:'Race: City',2:'Race: Sierra Nevada',3:'Race: Peninsula',4:'Race: Napa Valley',5:'Race: Pacific Coast',6:'Transition / unused',7:'Opening credits',8:'Race loading',9:'Main menu',10:'Thrash course selection',11:'Mano-a-mano setup',12:'Mano-a-mano courses',13:'Network chat presets',14:'Character selection',15:'Street / career hub',16:'Restroom / options hub',17:'Jukebox',18:'Configure input',19:'Graphics options',20:"Olley’s bike shop",21:'Rat bike showroom',22:'Sport bike showroom',23:'Super bike showroom',24:'Bike detail / purchase',25:'Der Panzer Klub',26:'Rider conversation',27:'Big Game course selection',28:'Wrecked result',29:'Busted result',30:'Won / qualified result',31:'Lost result',32:'Level advancement',33:'Penalty / game-over message',34:'Race standings',35:'Final credits'}
DYNAMIC_COUNTS={16:0x474668,17:0x47466c,18:0x465fb0,19:0x474670,11:0x46ca88,13:0x46ce80}
CHARACTERS=['AXLE','BOSE','CYDNEY','JON','PEARL','RHONDA','JIM','MIKE']
BIKES=['RAT/BANZAI','RAT/CORSAIR','RAT/KAMIKAZE','RAT/PERRO','RAT/KILLER','SPORT/DIABLO','SPORT/KAMIKAZE','SPORT/STILETTO','SPORT/DMG','SPORT/PERRO','SUPER/BANZAI','SUPER/DIABLO','SUPER/STILETTO','SUPER/CORSAIR','SUPER/KAMIKAZE']
POPUPS=[('mode',0x4735c0,4),('level',0x4736a0,6),('name',0x4737f0,2),('resolution',0x474240,4),('biker-list',0x474320,3),('videos',0x4743c8,4),('jukebox-video',0x473fa0,3)]+[(f'jukebox-tracks-{i+1}',a,n) for i,(a,n) in enumerate([(0x473b38,3),(0x473be0,2),(0x473c50,4),(0x473d30,5),(0x473e48,3),(0x473ef0,3)])]

def resolve_caption(strings, slot):
    if slot==0:return ''
    if slot==1:return 'Dynamic runtime text'
    text=strings.get(slot,f'String {slot}')
    for _ in range(64):
        if not text.endswith('ÿ'):return text
        if len(text)<2:raise ValueError('invalid string continuation')
        code=text[-2].encode('cp1252')[0];signed=code if code<128 else code-256
        text=text[:-2]+strings.get(3000+signed,'[unresolved continuation]')
    raise ValueError('string continuation depth')

def menu_items(exe, address, count, strings):
    if count>64: raise ValueError('menu count outside limit')
    items=[]
    for i in range(count):
        raw=exe.va(address+i*56,56)
        target,caption,x,y,w,h=struct.unpack_from('<iiiiii',raw,4)
        items.append(dict(index=i,target=target,caption_slot=caption,caption=resolve_caption(strings,caption),rect=[x,y,w,h],nav=list(raw[28:36]),flags=struct.unpack_from('<I',raw,36)[0],enabled=bool(raw[40]),handler=f'0x{struct.unpack_from("<I",raw,44)[0]:08x}',description_slot=struct.unpack_from('<i',raw,48)[0]))
    return items

def screens(exe, strings):
    result=[]
    for i in range(36):
        count,address,title,fmt,_=struct.unpack('<IIIII',exe.va(0x46a390+i*20,20))
        if i in DYNAMIC_COUNTS: count=struct.unpack('<I',exe.va(DYNAMIC_COUNTS[i],4))[0]
        result.append(dict(id=f'RR-{i:02X}',original_index=i,name=SCREEN_NAMES[i],title_slot=title,title=strings.get(title,''),items=menu_items(exe,address,count,strings) if address and count else [],origin='retail-table',dynamic_layout=i in [8,13,26,33,34],note='Stored table; handlers may mutate content/layout. Not a captured retail screen.'))
    return result

def group(path):
    p=path.upper()
    if '/HORIZONS/' in p or 'CLOUD.BOB' in p:return 'world-context'
    if p.endswith('.BOB'):return 'hud'
    for prefix,category in [('CHARS/','characters'),('POSTCARD/','courses'),('SHOWROOM/','showroom'),('BIKES/','bike-detail'),('RESULTS/','results'),('RESTROOM/','options-backgrounds'),('STREET/','hub-insets')]:
        if prefix in p:return category
    return 'menu-backgrounds'

def bob_size(path, length):
    p=path.upper(); n=Path(p).name
    if n.endswith('DASH.BOB'): width=640 if n[1:3]=='HI' else 320
    elif n=='BIGMETER.BOB':width=190
    elif n=='LILMETER.BOB':width=95
    elif n=='MATTE.BOB':width=640
    elif n=='BIGCLOUD.BOB':width=512
    elif n=='LILCLOUD.BOB':width=256
    elif '/HORIZONS/' in p:width=360 if n.startswith('L') else 720
    else:raise ValueError('unknown raw BOB stride')
    if length % width or not 0<length//width<=4096:raise ValueError('invalid BOB payload')
    return width,length//width

def decode_bob(data, size, palette):
    from PIL import Image
    if len(palette)!=1024 or len(data)!=size[0]*size[1]:raise ValueError('invalid indexed image')
    im=Image.new('RGBA',size)
    im.putdata([tuple(palette[v*4:v*4+3])+(255 if v else 0,) for v in data])
    return im

def external_output(path):
    path=path.resolve()
    if path==ROOT or ROOT in path.parents: raise ValueError('retail references must stay outside repository')
    return path

def catalog(install, output, video_thumbnails=True):
    from PIL import Image, ImageDraw, ImageFont, ImageOps
    output=external_output(output)
    registry=json.loads((ROOT/'tools/ui/source_ids.json').read_text())
    for path,digest in registry['input_sha256'].items():
        if hashlib.sha256((install/path).read_bytes()).hexdigest()!=digest:raise ValueError('unsupported reference build/input: '+path)
    output.mkdir(parents=True,exist_ok=True)
    (output/'reference-images').mkdir(exist_ok=True)
    images=[];failures=[]; palette=(install/'DATA/PALETTE.RAW').read_bytes()
    source_files=sorted(p for p in (install/'IMAGES').rglob('*') if p.is_file())
    for index,path in enumerate(source_files):
        rel=path.relative_to(install).as_posix();raw=path.read_bytes(); category=group(rel)
        if path.suffix.upper()=='.RRI':
            im=Image.open(io.BytesIO(raw));im.load()
            if max(im.size)>4096:raise ValueError('oversized RRI')
            im=im.convert('RGB');method='JPEG decode; not original IJG/GDI quantization parity'
        elif path.suffix.upper()=='.BOB':
            im=decode_bob(raw,bob_size(rel,len(raw)),palette);method='raw indexed pixels, palette0 transparent; opaque rendering is screen-dependent'
        else:raise ValueError(f'uncataloged image type: {rel}')
        ident=registry['images'][rel];preview=f'reference-images/{ident}.png';im.save(output/preview)
        images.append(dict(id=ident,source=rel,category=category,width=im.width,height=im.height,sha256=hashlib.sha256(raw).hexdigest(),preview=preview,replacement=f'replacements/{ident}.png',scope='context-only' if category=='world-context' else 'ui',status='needs-original-art',decode=method,confidence='inferred-unused-cloud-stride' if 'CLOUD' in rel else 'decoded-source',recommended_master=[im.width*4,im.height*4]))
    exe=PE((install/'ROADRASH.EXE').read_bytes());english=PE((install/'TEXT/ENU.DLL').read_bytes()).strings()
    menu=screens(exe,english);popups=[dict(id=f'RR-POP-{i+1:02}',name=name,items=menu_items(exe,a,n,english)) for i,(name,a,n) in enumerate(POPUPS)]
    pe_images=[];resource_stats={};dialogs=[]
    for file in [install/'ROADRASH.EXE',install/'RASHME.EXE']+sorted((install/'TEXT').glob('*.DLL')):
        if file.read_bytes()[:2]!=b'MZ':continue
        pe=PE(file.read_bytes()); entries=pe.resources();resource_stats[file.relative_to(install).as_posix()]=dict(Counter(str(p[0]) for p,_ in entries))
        for path,raw in entries:
            if path[0]==5:
                dialog=dialog_template(raw);dialog.update(id=f'RR-WIN-{file.stem}-{path[1]}',source=file.relative_to(install).as_posix(),resource_path=path,sha256=hashlib.sha256(raw).hexdigest(),name=f'Windows dialog {path[1]} ({file.stem})');dialogs.append(dialog)
            if path[0] not in [1,2,3]:continue
            ident=registry['embedded'][file.relative_to(install).as_posix()+'#'+'/'.join(map(str,path))]
            try:
                if path[0]==2:
                    from PIL.BmpImagePlugin import DibImageFile
                    im=DibImageFile(io.BytesIO(raw)).convert('RGBA')
                else:
                    dib=raw[4:] if path[0]==1 else raw
                    w,h=struct.unpack_from('<ii',dib,4);h//=2
                    if not 0<w<=256 or not 0<h<=256:raise ValueError('invalid icon size')
                    planes,bits=struct.unpack_from('<HH',dib,12)
                    header=struct.pack('<HHHBBBBHHII',0,1,1,w%256,h%256,0,0,planes,bits,len(dib),22)
                    im=Image.open(io.BytesIO(header+dib)).convert('RGBA')
                preview=f'reference-images/{ident}.png';im.save(output/preview)
                pe_images.append(dict(id=ident,source=file.relative_to(install).as_posix(),resource_path=path,category='application-icons',width=im.width,height=im.height,sha256=hashlib.sha256(raw).hexdigest(),preview=preview,replacement=f'replacements/{ident}.png',scope='ui',status='needs-original-art',decode='PE resource icon/cursor/DIB',confidence='decoded-source',recommended_master=[im.width*4,im.height*4]))
            except (ValueError,OSError,struct.error) as error:failures.append(dict(source=str(file.relative_to(install)),resource=path,error=str(error)))
    images+=pe_images
    videos=[]
    (output/'video-thumbnails').mkdir(exist_ok=True)
    for index,path in enumerate(sorted((install/'VIDEO').rglob('*.AVI'))):
        info=json.loads(subprocess.check_output(['ffprobe','-v','error','-show_entries','format=duration:stream=codec_type,width,height','-of','json',str(path)],text=True));v=next(s for s in info['streams'] if s['codec_type']=='video');duration=float(info['format']['duration']);ident=registry['videos'][path.relative_to(install).as_posix()];preview=f'video-thumbnails/{ident}.png'
        if video_thumbnails:
            subprocess.run(['ffmpeg','-v','error','-y','-ss',str(min(1.,duration*.25)),'-i',str(path),'-frames:v','1','-vf','scale=320:-1',str(output/preview)],check=True,stdout=subprocess.DEVNULL)
        videos.append(dict(id=ident,source=path.relative_to(install).as_posix(),width=v['width'],height=v['height'],duration_seconds=duration,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),preview=preview if video_thumbnails else None,status='optional-new-cutscene-or-static-card',scope='video-reference'))
    fonts=[dict(source=f'TEXT/{n}.DLL',role=role,sha256=hashlib.sha256((install/f'TEXT/{n}.DLL').read_bytes()).hexdigest(),exported=False,status='choose-original-or-licensed-replacement') for n,role in [('BADLOC','display / race banners'),('FUTR','regular UI text'),('FUTB','bold UI text'),('FUTD','demibold UI text')]]
    languages=[]
    for n in ['ENU','DEU','FRA','ESP','ITA']:
        p=install/f'TEXT/{n}.DLL';languages.append(dict(source=p.relative_to(install).as_posix(),string_resources=len(PE(p.read_bytes()).strings()),sha256=hashlib.sha256(p.read_bytes()).hexdigest()))
    # Six images per research sheet; full resolution remains available individually.
    (output/'contact-sheets').mkdir(exist_ok=True)
    font_path=next((p for p in ['/usr/share/fonts/google-noto/NotoSans-Regular.ttf','/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf'] if Path(p).exists()),None)
    font=ImageFont.truetype(font_path,13) if font_path else ImageFont.load_default()
    sheets=[]
    for start in range(0,len(images),6):
        sheet=Image.new('RGB',(1200,700),'#20252b');draw=ImageDraw.Draw(sheet)
        for slot,record in enumerate(images[start:start+6]):
            x=(slot%3)*400;y=(slot//3)*350;pic=Image.open(output/record['preview']).convert('RGBA');pic.thumbnail((388,290));sheet.paste(pic,(x+(400-pic.width)//2,y+8+(290-pic.height)//2),pic)
            draw.text((x+10,y+303),record['id']+' · '+record['category'],font=font,fill='white');draw.text((x+10,y+325),record['source'].replace('IMAGES/',''),font=font,fill='#b9c2cc')
        name=f'contact-sheets/UI-SHEET-{start//6+1:03}.jpg';sheet.save(output/name,quality=90);sheets.append(dict(path=name,ids=[r['id'] for r in images[start:start+6]]))
    metadata=dict(schema_version=1,reference_commit=REFERENCE_COMMIT,input_sha256=registry['input_sha256'],images=images,screens=menu,popups=popups,windows_dialogs=dialogs,videos=videos,fonts=fonts,languages=languages,pe_resource_types=resource_stats,failures=failures,sheets=sheets,counts=dict(image_files=len(source_files),images=len(images),ui_images=sum(r['scope']=='ui' for r in images),context_images=sum(r['scope']=='context-only' for r in images),retail_screen_slots=len(menu),popups=len(popups),videos=len(videos),windows_dialog_resources=len(dialogs),unique_windows_dialogs=len({d['sha256'] for d in dialogs})),limits=['stored menu tables are not complete dynamic handler behavior','no retail execution or bit-exact GDI render claim','cloud stride is inferred and not adopted by UI','all original media is local reference-only; no redistribution permission established'])
    (output/'catalog.json').write_text(json.dumps(metadata,indent=2)+'\n')
    (output/'english-menu-reference.json').write_text(json.dumps(dict(screens=menu,popups=popups),indent=2)+'\n')
    return metadata

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--install',type=Path,required=True);parser.add_argument('--output',type=Path,required=True);parser.add_argument('--no-video-thumbnails',action='store_true');args=parser.parse_args()
    result=catalog(args.install.resolve(),args.output,not args.no_video_thumbnails)
    print(json.dumps(dict(counts=result['counts'],failures=result['failures'],output=str(args.output.resolve())),indent=2))
    if result['failures']:raise SystemExit('Some embedded resources need review; see failures')
if __name__=='__main__':main()
