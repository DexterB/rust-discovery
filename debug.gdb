# Connect to remote target
target remote :3333
# Flash the application
load
# Set the break point on entry to main
break main
continue