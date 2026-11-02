import hashlib
import json
import unittest
import struct
from pathlib import Path
from group_building_districts import build,OUT,KIT
from measure_reference_districts import resources,course_loads


class DistrictTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.catalog=build();cls.assets={r['id']:r for r in cls.catalog['assets']}

    def test_exactly_one_primary_group_per_actual_asset(self):
        assigned=[i for g in self.catalog['districts']for i in g['asset_ids']]
        self.assertEqual(len(assigned),146)
        self.assertEqual(len(set(assigned)),146)
        self.assertEqual(set(assigned),set(self.assets))
        self.assertEqual(len(self.catalog['districts']),13)
        self.assertEqual(json.loads((OUT/'districts.json').read_text()),self.catalog)

    def test_similar_names_do_not_mix_facades_structures_and_surfaces(self):
        for identity,group in [('BUSSTOP2','roadside_services'),('BUSS1','commercial_high_street'),('CHINGAT2','chinatown'),('BARND2','farms_agriculture'),('TOP1A','shared_roof_edges'),('TUNN31RW','retaining_tunnels')]:
            self.assertEqual(self.assets[identity]['primary_district'],group)
        self.assertEqual(self.assets['TOP']['asset_type'],'surface_module')
        self.assertEqual(self.assets['CHINGAT2']['asset_type'],'structure')

    def test_original_course_eligibility_is_not_inferred_from_district(self):
        courses={c['code']:set(c['asset_ids'])for c in self.catalog['courses']}
        for code,identities in courses.items():
            self.assertEqual(identities,{i for i,a in self.assets.items()if code in a['original_course_family_load_matches']})
        for identity in ['CHINA12','CHINA17','CHINGAT2']:
            self.assertEqual(self.assets[identity]['original_course_family_load_matches'],['CITY'])
        for identity in ['TUNN31RW','TUNN1LIP']:
            self.assertEqual(self.assets[identity]['original_course_family_load_matches'],['CANYN'])
        self.assertFalse(self.catalog['district_labels_are_original'])
        self.assertGreater(len(self.assets['BUSSTOP2']['original_course_family_load_matches']),1)

    def test_grouped_links_and_relative_manifests_preserve_original_mesh_bytes(self):
        count=0
        for group in self.catalog['districts']:
            folder=OUT/group['id'];manifest=json.loads((folder/'manifest.json').read_text())
            self.assertTrue((folder/'contact-sheet.png').is_file())
            for asset in manifest['assets']:
                self.assertTrue((folder/asset['render']).is_file())
                for lod in asset['lods']:
                    target=folder/lod['file'];link=folder/'models'/target.name
                    self.assertTrue(link.is_symlink())
                    self.assertEqual(link.resolve(),target.resolve())
                    self.assertEqual(hashlib.sha256(target.read_bytes()).hexdigest(),lod['sha256']);count+=1
        self.assertEqual(count,438)

    def test_family_loads_exclude_unload_requests(self):
        payload=bytearray(68+24+4*12);payload[:4]=b'SGSR';struct.pack_into('<I',payload,8,1);struct.pack_into('<I',payload,28,68)
        payload[68:72]=b'MSRR';struct.pack_into('<II',payload,72,72,4)
        for k in range(4):
            row=92+k*12;struct.pack_into('<I',payload,row,k*10);payload[row+4]=k+1;struct.pack_into('<I',payload,row+8,100+k)
        data=bytearray(24)+payload;table=len(data);data+=bytearray(48);data[:4]=b'CRSR';data[12:16]=b'LBTR';struct.pack_into('<I',data,16,table)
        data[table:table+4]=b'LBTR';struct.pack_into('<I',data,table+8,1);data[table+16:table+20]=b' SGS';struct.pack_into('<III',data,table+20,1,24,len(payload))
        self.assertEqual(course_loads(bytes(data))['loaded_family_ids'],[100,102])
        struct.pack_into('<I',data,table+8,2)
        with self.assertRaises(ValueError):resources(bytes(data))

    def test_resource_reader_rejects_non_resource_input(self):
        with self.assertRaises(ValueError):resources(b'not a resource')


if __name__=='__main__':unittest.main()
