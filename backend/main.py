#!/usr/bin/python
from meals import current_meal, Meal
from flask import Flask, jsonify

app = Flask(__name__)

@app.route("/whatmeal")
def index():
    return jsonify({
        "current_meal": current_meal(),
    })

if __name__ == "__main__":
    app.run()
