# srv-watch
srv-watch is a cli client to watch videos on srv-hosts. it's built to be script-capable, with all interactive prompts explicitly opt-in with a `-i` flag. currently there is no support for any interactive prompts.

## usage
the follow command adds a feed to your following list
```bash
srv-watch follow https://something.com/feed.xml
```

the following command lists all the channels you're following
```bash
srv-watch following
```

then to see the posts of a feed you use the videos subcommand followed by the 0-based index from the following list
```bash
srv-watch following videos 2
```

to play one of those videos, use the play subcommand with the 0-based index on the video list
```bash
srv-watch following videos play 0
```
