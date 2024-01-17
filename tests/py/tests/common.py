from build import Root
from wasmtime import Store
import build.exports as wit

def instantiate():
    store = Store()
    root = Root(store)
    return root, store
