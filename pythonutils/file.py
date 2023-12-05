from pathlib import Path

INPUT_FILES_PATH = Path('./inputs/')
DAY_FILES_PATH = Path('./days/')


def create_day_input_file_path(day_number):
    path = Path(INPUT_FILES_PATH / f'{day_number}' / 'input.txt')
    path.parent.mkdir(parents=True, exist_ok=True)
    return path


def get_day_folder_path(day_number):
    return Path(DAY_FILES_PATH / f'day_{day_number}')


def get_sample_files_for_day(day_number) -> list[Path]:
    return list(get_day_folder_path(day_number).glob('sample_*.txt'))
