#!/usr/bin/python
from meals import current_meal
from flask import Flask, jsonify, request
import db

app = Flask(__name__)

@app.route("/stats")
def getstats():
    curr_meal = current_meal()
    res = {
        "currentMeal": db.mealtostr(curr_meal),
    }
    if curr_meal is not None:
        (scoresum, nvotes) = db.current_stats()
        res["scoresTotal"] = scoresum
        res["numVotes"] = nvotes

    return jsonify(res)

@app.route("/vote", methods = ["POST"])
def addvote():
    bodydata = request.get_json()
    if bodydata is None:
        return ("Bad request", 400)

    if not "score" in bodydata:
        return ('"score" field is missing', 400)

    submitted_score = bodydata["score"]

    if not isinstance(submitted_score, int):
        return ("Score is not an integer", 400)

    if submitted_score >= 0 and submitted_score <= 10:
        curr = current_meal()
        if curr is None:
            return ("No meal in progress", 400)
        v = db.Vote(curr, submitted_score)
        db.submit_vote(v)
        return ("Vote submitted", 200)
    else:
        return ("Score is out of range", 400)

@app.after_request
def after_request(resp):
    resp.headers.add("Access-Control-Allow-Origin", "*")
    resp.headers.add("Access-Control-Allow-Methods", "POST, GET")
    resp.headers.add("Access-Control-Allow-Headers", "*")
    return resp

if __name__ == "__main__":
    app.run(port=8080, host="0.0.0.0")
