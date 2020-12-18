def swap_op(str_):
    """Swap all '+' <--> '*' """
    tmp = str_.replace('*', '#')
    tmp = tmp.replace('+', '*')
    return tmp.replace('#', '+')


def wrap_digit(str_):
    """Wrap all digits with SwapInt constructor"""
    return "".join([f"SwapInt({c})" if c.isdigit() else c for c in str_])


class SwapInt:
    """int wrapper with add and mul operators swapped."""
    def __init__(self, val):
        self.val = val

    def __mul__(self, other):
        return SwapInt(int.__add__(self.val, other.val))

    def __add__(self, other):
        return SwapInt(int.__mul__(self.val, other.val))


expressions = open("input/18/input", "r")
expressions = expressions.readlines()
print(sum(eval(wrap_digit(swap_op(expr))).val for expr in expressions))
