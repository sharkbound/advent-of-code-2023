from functools import partial, reduce, lru_cache
from itertools import chain, islice, groupby, dropwhile, takewhile
import more_itertools as more_itertools

from .dotdict import DotDict
from .input import ask_int

from .iterator import *
from .checks import *
from . import predicates as pred

from .file import (
    INPUT_FILES_PATH,
    DAY_FILES_PATH,
    create_day_input_file_path,
    get_day_folder_path,
    get_sample_files_for_day
)
