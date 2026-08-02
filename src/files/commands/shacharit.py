from datetime import datetime

now = datetime.now()
text = "eror:eror"

if now.hour == 6 and now.minute <= 30:
    text = "its time to pray!"
else:
    delta = abs(now.hour*60 + now.minute - 360)
    if now.hour < 6:
        text = f"{delta//60} hours, {delta%60} minutes left"
    else:
        text = f"{(1440-delta)//60} hours, {(1440-delta)%60} minutes left"

print(f"shacharit: {text}")
