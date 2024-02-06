# app events simple analysis

This folder contains an example data set (taken from me julian lazaras) of how app usage can 
be used to perform more complex behavioral analysis, in particular this shows what app I most frequently check

## What the data shows

This data has been collected between may and june of 2023 and shows my ap usage over this time
the results of the query used are shown in appsused.txt that shows app package name, first name, last name, email,
and how many events were recorded within this month for that particular app.

## scripts

appsused.sql:
this script performs the query that generated this report

dump.sql:
this contains the example dataset as well as the database layout currently prototyped.

## why this matters?
not only does this information indicate what apps I favor, but it also shows how I interact with my device
purely based on this information one can determine that I prefer secure messaging apps
(things that may be more cutting edge) it also shows that I interacted with one particular app substantially more
than any other. further more the nature of the app (signal or package org.thoughtcrime) is a messaging app
but unlike the second most used app (discord or com.discord) it is not intended for, nor does it implement group
chat functionality. This indicates that I have a need for private communications with a limited number of individuals
and that my social interactions in the digital space are ones I may prefer to keep confidencial.

in addition the second most used app (excluding android itself, and the app launcher) is discord
which can profile me as someone who participates in community forum cultures based around isolated groups (servers)
this is further compounded by slack being high on the list. judging by tinder and grinndr also being high on the list,
I may be looking for romantic, but more likely sexual interactions, which is supported by the high usage of signal
(a need for confidentiality) as well as firefox being more used than chrome.

As for the openmhealth app, currently that only says I'm working a lot on developing the app (as it isn't a 
mainstream app at this time) which indicates I am an app developer. (which I am) this would make sense given
the backdrop of signal, discord, and firefox (while none of these apps used alone are conclusive being used together
does server to demonstrate a particular picture)

all of this can be inferred simply from one months worth of data, and only from a relatively primitive query, that
doesn't account for variables such as app launch frequency, time spent in app, time of day interacting with the app 
or events percipitating interaction (such as receinving a phone call, a grade change, or a text message being sent)
to say nothing of what I am doing in each app, or how often I'm using the keyboard (which is currently tracked)

final food for thought: fairly biased because I know what I'm doing in signal (I plan also to use libsignal
to get signal usage stats later) the fact that I'm using signal so much in favor of any other app by a large margin
on top of the use of grindr and tinder paints a picture of having a few stronger connections, wanting to branch out and meet people,
while also being fairly isolated and given my gender more prone to sexual desire that results from a lack of engagement,
or activity in the world around me. while inconclusive in it of itself, the more data collected from alternative sources
the easier it becomes to demonstrate high degrees of correlation between certain app usage, and certain behaviors
desires, and needs.
