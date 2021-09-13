from enum import Enum
import datetime as dt

class Meal(Enum):
    Breakfast = 1
    Brunch = 2
    Lunch = 3
    Dinner = 4

EDT = dt.timezone(-dt.timedelta(hours=4))

MON = 0
TUE = 1
WED = 2
THU = 3
FRI = 4
SAT = 5
SUN = 6

def current_meal():
    now = dt.datetime.now(tz=EDT)
    weekday = now.weekday()
    now = now.time()

    # breakfast
    if weekday != SAT and weekday != SUN:
        if now > dt.time(7, 0) and now < dt.time(10, 30):
            return Meal.Breakfast

    # brunch
    if weekday == SAT or weekday == SUN:
        if now > dt.time(9, 30) and now < dt.time(14, 30):
            return Meal.Brunch

    # lunch
    if weekday != FRI:
        if now > dt.time(11, 0) and now < dt.time(16, 0):
            return Meal.Lunch
    else:
        if now > dt.time(11, 0) and now < dt.time(17, 0):
            return Meal.Lunch

    # dinner
    if now > dt.time(17, 0) and now < dt.time(20, 0):
        return Meal.Dinner

    return None
