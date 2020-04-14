#!/usr/local/bin/python3
#
# Shows graphs of COVID-19 in Massachusetts for:
#
# - Total number of cases
# - Daily change in number of cases
#
# Inspired by this post
#
# https://towardsdatascience.com/modeling-logistic-growth-1367dc971de2
#
import math
import numpy as np
import pandas as pd
import scipy.optimize as optim
import matplotlib.pyplot as plt
import datetime

# A logistic curve has the form
#
# y(t) =  c / 1 + a * e ^ bt
#
# Where
# t is the time
# y(t) is the number of cases a time t
# c is the limiting or maximum value of y
# b is the growth factor and has to be larger than 0
# a is the maximum value c - initial number infected.  In this case 1 person
# is infected so a = c - 1 = 1000 - 1 = 999
def logistic(t, a, b, c):
    return c / (1 + a * math.exp(-b * t))

# Read in raw data
data = pd.read_csv("ma_coronavirus_data.csv")

#
# Create x and y axes for first plot which is total number of cases
#

# Convert pandas Data frame to numpy array
mass_case_y = np.array(data['Total Cases'])
print("Number of days: " + str(len(mass_case_y)))
mass_case_x = range(0, len(mass_case_y))

# Print the start date and today for reference
today = datetime.datetime.now()
time_delta = datetime.timedelta(days = len(mass_case_y))
beginning = today - time_delta
print("Start date: " + str(beginning))
print("End date: " + str(today))

#
# Create x and y axes for daily change rate in number of cases
#
change_y = []

# Calculate how many new cases per day
for i in range(1, len(mass_case_y)):
    change_y.append(mass_case_y[i] - mass_case_y[i - 1])
change_x = range(0, len(mass_case_y) - 1)

#
# Create logistic model
#

# Randomly initialize initial values for a, b, c
p0 = np.random.exponential(size=3)

# Min and max values for a, b, c
bounds = (0, [6999999, 3, 7000000])

# Need to call np.vectorize on the logistic function so we don't get the error
# TypeError: Only Size-1 Arrays Can be Converted to Python Scalars
logistic2 = np.vectorize(logistic)

# Now that we've initialized the initial values and the min/max, call
# curve_fit to fit the optimal values of a, b and c based on the actual
# data.  curve_fit returns the paraemters and a 2d covariance matrix between
# the parameters
(a,b,c),cov = optim.curve_fit(logistic2, mass_case_x, mass_case_y, bounds=bounds, p0=p0)

# Now create the x and y data passing the parameters we got from the curve_fit
# to the actual logistic function
model_x = range(0, len(mass_case_y) * 2)
logistic_y = [logistic(i, a, b, c) for i in model_x]

# Create the subplots now
fig, ax = plt.subplots(nrows=2, ncols=2)
ax[0, 0].plot(mass_case_x, mass_case_y)
ax[0, 0].set_title("Total Cases")
ax[0, 1].plot(change_x, change_y)
ax[0, 1].set_title("Daily New Cases")
ax[1, 0].plot(model_x, logistic_y)
ax[1, 0].set_title("Model")
plt.show()
