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

def create_today():
    todaypath = get_todaypath()
    makedirs(todaypath)
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
    mealfile = get_todaypath() / mealtostr(vote.meal)
    with open(mealfile, "a") as mf:
        mf.write(f"\n{vote.timestamp.hour}:{vote.timestamp.minute}:{vote.timestamp.second} {vote.score}")

# score of the current meal
def current_score():
    mealfile = get_todaypath() / mealtostr(current_meal())
    lines = Path(mealfile).read_text().split("\n")[1:]

    n_scores = 0
    tot = 0
    for line in lines:
        tot += int(line.split(" ")[1])
        n_scores += 1

    if n_scores == 0:
        return None
    return tot / n_scores
