#!/usr/bin/python
from meals import current_meal, Meal
from flask import Flask, jsonify
import db

app = Flask(__name__)

@app.route("/stats")
def index():
    curr_meal = current_meal()
    res = {
        "current_meal": db.mealtostr(curr_meal),
    }
    if curr_meal is not None:
        (scoresum, nvotes) = db.current_stats()
        res["scores_sum"] = scoresum
        res["n_votes"] = nvotes

    return jsonify(res)

if __name__ == "__main__":
    app.run()
