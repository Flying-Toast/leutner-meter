#!/usr/bin/python
from meals import current_meal, Meal
from flask import Flask

app = Flask(__name__)

@app.route("/")
def index():
    return "aaaaaa"

if __name__ == "__main__":
    app.run()
