import unittest
from audit import permitted
class LicenseExpressionTests(unittest.TestCase):
    def test_choices_and_obligations(self):
        for expression in ['MIT OR GPL-2.0-only','(MIT OR Apache-2.0) AND Unicode-3.0','Apache-2.0 WITH LLVM-exception','MIT/Apache-2.0']:
            self.assertTrue(permitted(expression),expression)
        for expression in ['GPL-2.0-only AND MIT','unknown','MIT AND','MIT OR (GPL-2.0-only','MIT WITH unknown-exception']:
            self.assertFalse(permitted(expression),expression)
if __name__=='__main__': unittest.main()
