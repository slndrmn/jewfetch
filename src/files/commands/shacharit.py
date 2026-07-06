from datetime import datetime

now = datetime.now()

if now.hour == 6 and now.minute <= 30:
    print("its time to pray!")
else:
    delta = abs(now.hour*60 + now.minute - 360)
    if now.hour < 6:
        print(f"{delta//60} hours, {delta%60} minutes left")
    elif now.hour == 6 and now.minute > 30:
        print(f"{(1440-delta)//60} hours, {(1440-delta)%60} minutes left")
    else:
        print(f"{(1440-delta)//60} hours, {(1440-delta)%60} minutes left")
