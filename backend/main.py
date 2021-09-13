#!/usr/bin/python
from meals import current_meal
from flask import Flask, jsonify, request
import db

app = Flask(__name__)

@app.route("/stats")
def getstats():
    curr_meal = current_meal()
    res = {
        "current_meal": db.mealtostr(curr_meal),
    }
    if curr_meal is not None:
        (scoresum, nvotes) = db.current_stats()
        res["scores_sum"] = scoresum
        res["n_votes"] = nvotes

    return jsonify(res)

@app.route("/vote", methods = ["POST"])
def addvote():
    try:
        submitted_score = int(request.form["score"])
        if submitted_score >= 0 and submitted_score <= 10:
            v = db.Vote(current_meal(), submitted_score)
            db.submit_vote(v)
            return ("Done", 200)
        else:
            return ("Bad vote", 400)
    except:
        return ("Bad vote", 400)

if __name__ == "__main__":
    app.run()
