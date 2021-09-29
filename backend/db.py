import datetime as dt
from pathlib import Path
from os import makedirs
from meals import Meal, current_meal

DB_ROOT = Path("db")

EDT = dt.timezone(-dt.timedelta(hours=4))

def get_todaypath():
    now = dt.datetime.now(tz=EDT)
    return DB_ROOT / f"{now.year}" / f"{now.month}" / f"{now.day}"

def mealtostr(m):
    if m == Meal.Breakfast:
        return "breakfast"
    if m == Meal.Brunch:
        return "brunch"
    if m == Meal.Lunch:
        return "lunch"
    if m == Meal.Dinner:
        return "dinner"
    else:
        return None

def create_today():
    todaypath = get_todaypath()
    try:
        makedirs(todaypath)
    except:
        pass
    open(todaypath / "breakfast", "a").close()
    open(todaypath / "brunch", "a").close()
    open(todaypath / "lunch", "a").close()
    open(todaypath / "dinner", "a").close()

class Vote:
    def __init__(self, meal, score):
        self.timestamp = dt.datetime.now(tz=EDT).time()
        self.meal = meal
        self.score = score
        pass

def submit_vote(vote):
    create_today()
    mealfile = get_todaypath() / mealtostr(vote.meal)
    (tot, n_scores) = get_stats(mealfile)
    write_stats(mealfile, tot + vote.score, n_scores + 1)

def get_stats(mealfile):
    line = Path(mealfile).read_text()
    if line == "":
        return (0, 0)
    parts = line.split(" ")

    return (int(parts[1]), int(parts[0]))

def write_stats(mealfile, tot, n_scores):
    with open(mealfile, "w") as mf:
        mf.write(f"{n_scores} {tot}")

def current_stats():
    create_today()
    mealfile = get_todaypath() / mealtostr(current_meal())
    return get_stats(mealfile)
