import unittest
from .common import instantiate, wit

class MyTestCase(unittest.TestCase):
    def test_revision(self):
        root, store = instantiate()
        crate = root.crate()
        self.assertEqual(crate.revision(store), 15)

    def test_class(self):
        root, store = instantiate()
        crate = root.crate()
        self.assertEqual(crate.class_(store, "0"), wit.crate.MathClass.NORMAL)
