import datetime as dt
from pathlib import Path
from os import makedirs

DB_ROOT = Path("db")

EDT = dt.timezone(-dt.timedelta(hours=4))

def get_todaypath():
    now = dt.datetime.now(tz=EDT)
    return DB_ROOT / f"{now.year}" / f"{now.month}" / f"{now.day}"

def create_today():
    todaypath = get_todaypath()
    makedirs(todaypath.parent)
    open(todaypath, "a").close()
