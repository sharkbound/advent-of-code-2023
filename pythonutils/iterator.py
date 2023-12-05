import itertools
import re
import typing
from typing import Callable, Any

__all__ = [
    'ITER_END_MARKER',
    'iter_with_terminator',
    'IterEndMarker',
    'iter_flatten',
    'flatten',
    'get_all_ints',
    'build_mapping',
    'build_mapping_from_iter',
    'first_where',
    'first_where_not',
    'reverse_mapping',
    'format_map',
    'last_where',
    'last_where_not',
    'map_inner_elements',
    'filter_not',
]


class IterEndMarker:
    def __repr__(self):
        return '<IterEndMarker>'

    def __bool__(self):
        return False


ITER_END_MARKER = IterEndMarker()


def iter_with_terminator(iterable, end_marker=ITER_END_MARKER, include_end_marker=True, transform=None, predicate=None):
    if predicate is None:
        predicate = lambda x: True

    if transform is None:
        transform = lambda x: x

    # iterators return themselves if you call iter() on it, this is to ensure it's an iterator
    it = iter(iterable)

    while (value := next(it, end_marker)) != end_marker:
        if predicate(transformed := transform(value)):
            yield transformed

    if include_end_marker:
        yield end_marker


def iter_flatten(iterable, depth=None, transform=lambda x: x):
    if depth is not None and depth < 0:
        yield transform(iterable)
        return

    # strings and bytes are themselves iterable, so we dont want to include them
    if isinstance(iterable, typing.Iterable) and not isinstance(iterable, (str, bytes)):
        for x in iterable:
            yield from iter_flatten(x, depth=((depth - 1) if depth is not None else None))
    else:
        yield transform(iterable)


def flatten(iterable, depth=None, transform_items=lambda x: x, result_transform=tuple):
    return result_transform(iter_flatten(iterable, depth, transform_items))


def map_inner_elements(transform_item, iterable, result_transform=None):
    result_transform_func = (
        result_transform
        if result_transform is not None
        else type(iterable)
    )
    return result_transform_func(map(transform_item, iterable))


def get_all_ints(value, transform=iter):
    match value:
        case str() as s:
            return transform(map(int, re.findall(r'\d+', s)))
        case iterable if isinstance(iterable, typing.Iterable):
            return transform(map(int, iterable))
        case _:
            raise ValueError(f'cannot find ints from type: {type(value)}. value must be iterable!')


def build_mapping(*items, cls=dict):
    """
    builds a dict from pairs passed, each 2 values are interpreted as (key, value)
    """
    # check that it's an even length
    assert len(items) & 1 != 1, 'length of items must be even in build_dict(...)'
    return cls(zip(items[::2], items[1::2]))


def build_mapping_from_iter(iterable, cls=dict, fillvalue=None):
    """
    builds a dict from pairs passed, each 2 values are interpreted as (key, value)
    """
    it = iter(iterable)
    return cls(
        (v1, v2)
        for v1, v2 in itertools.zip_longest(it, it, fillvalue=fillvalue)
    )


def first_where(iterable, predicate=lambda x: True, default=None):
    return next(filter(predicate, iterable), default)


def first_where_not(iterable, predicate=lambda x: True, default=None):
    return next(filter(lambda x: not predicate(x), iterable), default)


def last_where(iterable, predicate=lambda x: True, default=None):
    val = default
    for item in filter(predicate, iterable):
        val = item
    return val


def last_where_not(iterable, predicate=lambda x: True, default=None):
    val = default
    for item in filter(lambda x: not predicate(x), iterable):
        val = item
    return val


def reverse_mapping(mapping):
    return type(mapping)(zip(mapping.values(), mapping))


def format_map(items, format_: str | Callable, transform: Callable[[Any], Any] = None, args=False, kwargs=False):
    if not callable(format_):
        format_ = format_.format_map if kwargs else format_.format

    if transform is None:
        transform = lambda x: x

    if args:
        formatter = lambda x: format_(*x)
    else:
        formatter = format_

    return transform(map(formatter, items))


def filter_not(iterable, predicate=lambda x: False):
    yield from filter(lambda x: not predicate(x), iterable)
