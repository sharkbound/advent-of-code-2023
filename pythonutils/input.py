def ask_int(prompt):
    while True:
        day = input(prompt)
        if not day.isnumeric():
            continue
        return int(day)
