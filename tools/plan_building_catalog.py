#!/usr/bin/env python3
"""Explicit per-reference mesh authoring descriptions from all reviewed 3D sheets.
These are art-direction/geometry parameters, not inferred completed mesh claims.
"""
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
FOLDER=ROOT/'art/buildings/mesh_assets'
SOURCE=json.loads((ROOT/'art/buildings/concept_3d/MANIFEST.json').read_text())
REF={name:r['file'] for r in SOURCE['records'] for name in r['ids_in_row_major_order']}
# id, material, storeys, columns, window, roof, entry, extras
SPECS='''
BUSS7|weathered_cream|2|2|bay|steep_gable|shop|slate,attic,blue_sign
BUSS8|weathered_cream|2|2|sash|flat|shop|green_awning,flower_boxes,gothic_sign
BUSS9|weathered_cream|3|2|deco|flat|shop|vertical_piers,pink_awning
CHINA12|weathered_cream|3|2|lattice|pagoda_green|shop|tiered_eaves,red_columns,lanterns,gold_sign
CHINA13|weathered_cream|3|2|lattice|pagoda_slate|shop|tiered_eaves,upturned_finials,pottery,gold_sign
CHINA14|green_plaster|3|3|shutters|flat|shop|tiered_eaves,balcony,lanterns,gold_sign
CHINA15|ochre_brick|3|3|shutters|flat|door|tiered_eaves,terracotta,red_columns
CHINA16|whitewashed_brick|3|2|doorlights|dutch|door|tiered_eaves,balconies,green_timber,flower_boxes
CHINA17|weathered_cream|3|2|lattice|flat|shop|tiered_eaves,balconies,green_timber,gold_sign
CLSTGA11|whitewashed_brick|2|2|sash|dutch|double_arch|ornate_storefront
CLSTGA12|red_brick|2|2|boarded|flat|shop|plank_awning,flower_boxes,sign_board
CLSTGA13|whitewashed_brick|2|3|sash|flat|shop|black_belt,ornate_storefront
CLSTGA14|weathered_cream|2|2|shutters|flat|door|ornate_cornice,red_trim
CLSTGA15|weathered_cream|2|3|shutters|flat|double_arch|green_shutters,stone_belt,vents
CLSTGA16|weathered_cream|2|3|sash|dutch|shop|stone_belt,sign_board
CLSTGA17|red_brick|2|1|gothic|gable_parapet|gothic_pair|corner_piers
CLSTGA18|ochre_brick|3|1|small|flat|door|relief_panels,black_shop
CLSTGA19|stone|2|3|gothic|steep_gable|gothic|tracery,rose_window
CLSTGA20|rust|2|2|boarded|flat|barn|corrugated,boarded_upper
CLSTGA21|stone|2|2|sash|battlement|double_arch|crest
OFFC1|blue_plaster|5|1|arched|flat|arch|balconies,floor_bands
OFFC10|ochre_brick|3|1|sash|flat|glazed|red_frames
OFFC11|blue_brick|3|1|wide|flat|arch|stone_belt
OFFC12|blue_brick|3|2|sash|flat|arch|stone_ground
OFFC13|blue_brick|4|2|sash|flat|door|corner_quoins
OFFC14|brown_stone|4|2|deco|flat|door|full_height_piers,wood_panels
OFFC15|whitewashed_brick|4|3|wide|flat|arch|blue_glass,floor_bands
OFFC16|red_brick|4|1|wide|flat|door|corner_piers,diamonds
OFFC17|weathered_cream|4|2|sash|flat|arch|stone_belt,wide_base_window
OFFC18|whitewashed_brick|4|1|sash|flat|arch|rusticated_ground,ornate_cornice
OFFC19|whitewashed_brick|3|3|slender|flat|door|ornate_storefront,crest
OFFC2|red_brick|6|2|arched|flat|arch|fanlight,stone_ground
OFFC20|whitewashed_brick|4|3|deco|flat|glazed|full_height_piers
OFFC21|whitewashed_brick|4|3|slender|flat|gothic|floor_bands,ornate_cornice
OFFC22|red_brick|5|1|wide|flat|door|balconies,side_door
OFFC5|blue_brick|4|1|small|flat|arch|stone_belt
RUNDWN11|weathered_cream|3|1|sash|pediment|shop|projecting_upper,small_lower_pair,red_sign
RUNDWN12|weathered_cream|3|2|bay|twin_gable|door|steps,railing,stone_base
RUNDWN13|red_brick|3|1|stacked|flat|window|projecting_upper,ornate_cornice
RUNDWN14|ochre_brick|3|1|bay|flat|door|offset_bay,side_slits
RUNDWN15|weathered_cream|2|2|sash|flat|side_door|arch_window,balcony,stone_base
RUNDWN16|whitewashed_brick|3|2|sash|flat|sealed|balconies,exposed_brick
RUNDWN17|ochre_brick|3|1|small|flat|door|stone_base,black_cornice
VICT11|blue_plaster|3|2|sash|flat|door|scroll_pediment,floor_corbels
VICT12|green_plaster|3|1|bay|steep_gable|barn|attic,red_door,fishscale
VICT13|blue_brick|3|1|bay|flat|arch|ornate_cornice,fanlight
VICT14|red_brick|3|1|sash|arched_pediment|arch|balconies,scrolls,attic
VICT15|red_brick|2|1|small|flat|boarded|plain_entry
VICT16|red_brick|3|1|turret_bay|turret|arch|steps,railing,attic
VICT17|red_brick|3|1|bay|flat|sealed|projecting_upper,planter
VICT18|brown_stone|3|1|sash|flat|sealed|picket_fence,planter
VICT19|ochre_brick|3|1|sash|flat|sealed|triangular_hood,red_trim,stone_base
VICT1A|ochre_brick|3|1|sash|flat|side_door|triangular_hood,red_trim,stone_base
VICT1B|ochre_brick|3|1|sash|flat|side_door|triangular_hood,red_trim,stone_base,boarded_lower
VICT20|ochre_brick|2|1|bay|gable_parapet|door|steps,brick_stair_walls,attic
VICT21|brown_stone|3|1|bay|flat|barn|heavy_bands
VICT22|ochre_brick|3|1|sash|arched_pediment|arch|balconies,scrolls,attic
VICT23|red_brick|3|1|sash|flat|sealed|plain_entry
VICT24|ochre_brick|3|1|bay|gable_parapet|arch|decorative_bay_panels,attic
VICT25|red_brick|3|1|turret_bay|flat|door|steps,brick_stair_walls,heavy_bands
VICT26|moss_brick|3|1|sash|flat|sealed|picket_fence,planter
VICT27|red_brick|3|2|sash|flat|passage|arched_passage,red_trim
VICT28|blue_plaster|3|1|bay|flat|garage|heavy_bands
VICT29|brown_stone|3|2|sash|steep_gable|side_door|balconies,gothic_door,arched_passage
VICT2A|ochre_brick|3|1|bay|gable_parapet|barn|decorative_bay_panels,attic,red_door
VICT2B|ochre_brick|2|1|bay|gable_parapet|door|steps,brick_stair_walls,attic,red_door
VICT31|ochre_brick|3|2|sash|flat|side_door|offset_windows,wood_panels
VICT32|stone|3|1|sash|flat|garage|stone_belt
VICT33|weathered_cream|3|3|sash|hip_front|side_door|gothic_door,terracotta,offset_windows
VICT34|red_brick|3|2|gothic|steep_gable|side_door|corner_piers,side_slits
VICT3A|blue_brick|3|1|bay|flat|arch|iron_gate,heavy_bands
VICT3B|purple_brick|3|1|bay|flat|garage|purple_ground,heavy_bands
VICT6A|red_brick|3|1|turret_bay|turret|arch|steps,railing,attic
VICT6B|red_brick|3|1|bay|gable_parapet|barn|attic,red_door
WHSE11|black_metal|2|1|none|flat|sealed|metal_cladding,rust_belts
WHSE12|whitewashed_brick|2|1|wide|flat|barn|wire_glass,black_barn
WHSE13|brown_stone|2|1|small|flat|sealed|stone_belts,slit_window
WHSE14|black_metal|2|2|small|flat|sealed|metal_cladding,wire_glass,red_brick_base
WHSE15|red_brick|2|1|wide|flat|garage|bollards
WHSE16|rust|2|1|none|flat|loading|corrugated,loading_platform
WHSE17|rust|2|1|sash|flat|loading|corrugated,red_frame,wire_glass
WHSE18|brown_stone|2|1|small|flat|loading|black_cornice,red_ground
WHSE19|weathered_cream|2|1|small|flat|sealed|black_belts
WHSE20|weathered_cream|3|2|small|flat|garage|vents,stone_belts
WHSE22|stone|3|1|small|flat|small|boarded_windows,metal_straps,red_brick_side
WHSE23|red_brick|2|1|loading|flat|loading|loading_platform,plank_awning
WRHS1|ochre_brick|4|1|industrial|flat|vent|full_height_grid
WRHS2|red_brick|3|1|industrial|flat|loading|boarded_upper
WRHS3|red_brick|3|1|industrial|flat|sealed|stone_belts
WRHS4|weathered_cream|3|2|arched|flat|sealed|ivory_frames
WRHS5|black_metal|2|3|none|flat|barn|star_bolts,metal_cladding,rust_belts
WRHS6|blue_plaster|2|1|wide|flat|barn|wire_glass,black_barn
WRHS7|stone|2|1|small|flat|sealed|stone_belts,slit_window
WRHS8|black_metal|3|2|small|flat|sealed|metal_cladding,boarded_windows,red_brick_base
'''
# Structure kind, width, depth, eaves/body height, distinctive geometry/features.
STRUCTURES='''
BARN12|barn|9|13|7|red_timber,raised_centre,lean_to,cupola,cross_braced_doors
BARN22|barn|8|11|5.5|old_timber,rust_roof,lean_to,fenced_pen
BARN32|shed|7|9|4.5|old_timber,open_work_bay,workbench
BARND2|barn|9|11|5.5|red_timber,clerestory,lean_to,side_hatch
BUSSTOP2|bus_shelter|3.5|2.0|3.0|clear_glass,bench,route_board,curved_roof
CARWASH2|shed|7|8|3.6|old_timber,open_bays,divider_walls
CHINGAT2|gate|8|2|7|pagoda_green,paired_stone_piers,upturned_finials,lattice
CHURCH2|church|9|18|7|white_siding,pointed_windows,bell_tower,spire
GASA2|gas_station|9|8|4|red_brick,forecourt_canopy,two_pumps,shop_glass
GASB2|gas_station|10|9|4|whitewashed_brick,wide_canopy,three_pumps,garage
GRNTNK2|silos|10|5|8|three_silos,conical_caps,ladder,small_silo
HAYSHED2|hay_shed|10|9|5|open_timber_frame,corrugated_roof,stacked_bales
HOUSEB2|house|7|9|5|white_siding,porch,railings,steep_roof
OUTH0|unused|0|0|0|none
OUTHOUS2|outhouse|1.6|1.8|2.6|red_timber,pitched_roof,door_latch
PHONSHK2|phone_shack|1.5|1.6|2.7|rough_planks,hardware,irregular_roofline
PORTLET2|portable_toilet|1.3|1.4|2.5|blue_plastic,rounded_cap,vents,hinges,latch
RUSTICA2|cabin|9|10|5|old_timber,rust_roof,porch,stove_pipe
RUSTICB2|cabin|9|10|5|white_siding,porch,fuel_pump,stove_pipe
SCHOOL2|school|9|13|6|white_siding,gabled_entry,tall_windows,flagpole
SHOPD2|grocery|7|8|4|whitewashed_brick,porch,green_trim,grocery_sign
SHOPE2|shop|7|7|3.6|whitewashed_brick,flat_roof,cassette_shop,storefront_shelves
SHOPICE2|ice_shop|8|8|4|white_siding,gable_roof,ice_freezer,storefront_shelves
SNCKSHP2|snack_shop|8|7|4|whitewashed_brick,canopy,serving_windows,snack_sign
TANKSEP2|septic_tank|3.6|3.6|3.5|black_ribbed_cylinder,inlet,outlet,lid
TOWERB2|stone_tower|4|4|8|tapered_stone,door,coping
WATRTNK2|water_tank|5|5|6|vertical_riveted_tank,conical_cap,ladder
WHTTANK2|fuel_tank|7|3.5|4|horizontal_cylinder,metal_supports,end_flanges
WINNERY2|winery|9|12|7|stone_walls,hip_roof,porch,arched_windows
'''
SURFACES={
'METL21CC':('metal_panel','riveted metal diagonal bracing and hinge strip'),
'METL21RC':('stone_panel','irregular dark stone masonry'),
'METL21RW':('vent_panel','concrete ventilation blocks over dark stone'),
'METL22CC':('metal_panel','riveted metal diagonal bracing with cylindrical hinge'),
'TOP':('wood_strip','horizontal old timber planks with bolts'),
'TOP1A':('metal_strip','green painted rusty edge strip'),
'TOP2A':('stone_strip','mossy low stone wall cap course'),
'TOP3A':('stone_strip','staggered mossy stone cap course'),
'TUN1SHTP':('aggregate_slab','exposed aggregate sloped concrete slab'),
'TUNN1LIP':('aggregate_strip','thick exposed aggregate tunnel lip'),
'TUNN21CC':('timber_panel','vertical timber wall with stone joints'),
'TUNN21RC':('brick_pier','dark brick square capped pier'),
'TUNN21RW':('brick_pier','brick/concrete capped pier with metal strapping'),
'TUNN22CC':('concrete_panel','horizontal concrete wall panel with coping'),
'TUNN31CC':('aggregate_slab','exposed aggregate concrete ground slab'),
'TUNN31RC':('concrete_pier','aggregate-faced pier with three bands'),
'TUNN31RW':('vent_pier','concrete pier with louver vent and bands'),
'TUNN32RC':('aggregate_panel','dark cracked wall panel with horizontal metal straps')}


def main():
    accepted=json.loads((FOLDER/'designs.json').read_text())['designs'][:6]
    designs=[]
    for d in accepted:
        designs.append(dict(**d,asset_type='facade',builder='accepted_first_sheet',reference_sheet='facades_01.png',review_note='User-liked first-six model retained without geometry changes'))
    for line in SPECS.strip().splitlines():
        identity,mat,storeys,columns,window,roof,entry,extras=line.split('|')
        n=int(storeys);width=3.2 if int(columns)<3 else 3.6
        height=3.5+(n-1)*2.6+.6
        designs.append(dict(id=identity,asset_type='facade',builder='catalog_facade',width=width,depth=6.,height=round(height,2),
                            masonry=mat,trim='oxblood' if 'red_trim' in extras else 'ivory',storeys=n,columns=int(columns),window=window,roof=roof,entry=entry,
                            features=extras.split(','),reference_sheet=REF[identity],review_note=f'{window} / {roof} / {entry}; {extras}'))
    for line in STRUCTURES.strip().splitlines():
        identity,kind,w,d,h,features=line.split('|')
        if identity not in REF:continue
        designs.append(dict(id=identity,asset_type='structure',builder=kind,width=float(w),depth=float(d),height=float(h),features=features.split(','),reference_sheet=REF[identity],review_note=features))
    for identity,(kind,note) in SURFACES.items():
        strip='strip' in kind
        designs.append(dict(id=identity,asset_type='surface_module',builder=kind,width=4 if strip or 'slab' in kind else 2,depth=.25 if not 'slab' in kind else 2.5,
                            height=.6 if strip else .22 if 'slab' in kind else 3,features=note.split(', '),reference_sheet=REF[identity],review_note=note))
    ids=[d['id'] for d in designs]
    if len(ids)!=146 or len(set(ids))!=146 or set(ids)!=set(REF):raise ValueError(f'Catalog mismatch: {len(ids)} missing {set(REF)-set(ids)}')
    designs.sort(key=lambda d:(d['reference_sheet'],list(REF).index(d['id'])))
    result=dict(schema_version=2,source='All 25 draft concept-render sheets inspected; explicit per-ID authoring parameters',designs=designs,
                asset_counts={kind:sum(d['asset_type']==kind for d in designs) for kind in ['facade','structure','surface_module']},
                status='Authoring plan only until model exports/validation/render review are complete')
    (FOLDER/'catalog_designs.json').write_text(json.dumps(result,indent=2)+'\n')
    print(result['asset_counts'])
if __name__=='__main__':main()
