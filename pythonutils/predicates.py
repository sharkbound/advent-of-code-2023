def eq_len(length):
    return lambda x: len(x) == length


def ne_len(length):
    return lambda x: len(x) != length


def le_length(length):
    return lambda x: len(x) <= length


def ge_length(length):
    return lambda x: len(x) >= length


def lt_length(length):
    return lambda x: len(x) < length


def gt_length(length):
    return lambda x: len(x) > length


def eq(value_to_match):
    return lambda x: x is value_to_match or x == value_to_match


def eq_ref(value_to_match):
    return lambda x: x is value_to_match


def contains(value_to_contain):
    return lambda x: value_to_contain in x


def not_contains(value_to_contain):
    return lambda x: value_to_contain not in x


def contains_eq_count(*values_to_contain, count=None):
    count = count if count is not None else len(values_to_contain)
    return lambda x: sum(1 for v in values_to_contain if v in x) == count


def contains_ne_count(*values_to_contain, count=None):
    count = count if count is not None else len(values_to_contain)
    return lambda x: sum(1 for v in values_to_contain if v in x) != count


def contains_all(*values_to_contain):
    return lambda x: all(v in x for v in values_to_contain)


def not_contains_all(*values_to_contain):
    return lambda x: all(v not in x for v in values_to_contain)


def combine(*preds):
    return lambda x: all(pred(x) for pred in preds)
