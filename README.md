# Early Bird Enforcer
This bot will only allow you to post in the hardcoded channels from 12am - 8am CDT. Any message outside of these times will be automatically deleted.

### Future Improvements
* Store channel_ids in a database to open up use for other servers
* Respect users' timezones
* Switch to comparing datetimes directly
* DM the user to shame them for dishonoring the sanctity of the early bird's channel

### Setting up
1. Add the bot to your server using the link generated from Discord's Developer Portal
2. Add the channel ID of the early-birds channel into the match statement
