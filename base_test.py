import test_sub

class Sub(test_sub.Base):
    def __init__(self):
        super().__init__()
        pass
    def super_call(self):
        print("Subclass Call.")

caller = test_sub.Caller(Sub())
caller.call_inner()
a = Sub()
a.super_call()
