import test_sub

class Sub(test_sub.Base):
    def __init__(self):
        pass
    def first_call(self):
        print("First Subclass Call.")
    def second_call(self):
        print("Second Subclass Call.")

a = Sub()
a.first_call()
a.second_call()
caller = test_sub.Caller(a)
caller.call_first_inner()
caller.call_second_inner()
