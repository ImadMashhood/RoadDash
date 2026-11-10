import unittest
from audit import permitted, permitted_dependency, metadata
from unittest.mock import patch
import json
class LicenseExpressionTests(unittest.TestCase):
    def test_router_dependency_intake_is_exact_and_not_global(self):
        package = dict(name='attohttpc', version='0.30.1', license='MPL-2.0',
                       source='registry+https://github.com/rust-lang/crates.io-index')
        self.assertTrue(permitted_dependency(package))
        self.assertFalse(permitted('MPL-2.0'))
        for key, value in [('name','another-library'),('version','0.30.2'),
                           ('license','GPL-3.0-only'),('source',None)]:
            self.assertFalse(permitted_dependency(dict(package, **{key:value})))
        with patch('audit.hashlib.sha256') as digest:
            digest.return_value.hexdigest.return_value = 'changed-license'
            self.assertFalse(permitted_dependency(package))
    def test_choices_and_obligations(self):
        for expression in ['MIT OR GPL-2.0-only','(MIT OR Apache-2.0) AND Unicode-3.0','Apache-2.0 WITH LLVM-exception','MIT/Apache-2.0']:
            self.assertTrue(permitted(expression),expression)
        for expression in ['GPL-2.0-only AND MIT','unknown','MIT AND','MIT OR (GPL-2.0-only','MIT WITH unknown-exception']:
            self.assertFalse(permitted(expression),expression)
class MetadataEncodingTests(unittest.TestCase):
    def test_cargo_utf8_names_do_not_use_windows_code_page(self):
        expected={"packages":[{"authors":["Łukasz"]}]}
        payload=json.dumps(expected,ensure_ascii=False).encode("utf-8")
        def windows_stdout(*args,**kwargs):
            return payload.decode(kwargs.get("encoding","cp1252")) if kwargs.get("text") else payload
        with patch("audit.subprocess.check_output",side_effect=windows_stdout):
            self.assertEqual(metadata(),expected)
if __name__=='__main__': unittest.main()
