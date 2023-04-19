# [Speech To Text Bot][invite]

Speech To Text Bot is a Discord bot that transcribes voice messages.

![image](https://user-images.githubusercontent.com/72335827/232255244-c36283ab-934b-4576-a98c-be3e4a96fea0.png)

## Commands

The bot supports the following commands:

- `/privacy`: Displays the bot's privacy policy.
- `/terms`: Displays the bot's terms of service.
- `/invite`: Displays an invite link to add the bot to your Discord server.
- `/help`: Displays a list of available commands.

In addition to the slash commands, the bot also supports a message context command. Simply right click the message and click `Apps -> Transcribe Message` and the bot will transcribe the message and send it in the same channel.

![image](https://user-images.githubusercontent.com/72335827/232306462-e4caab5e-aa54-4c60-b64f-fb2d24548838.png)

### Translating

1. The bot reads the user locale first to determine if its a non english locale if so it will translate the voice message to that locale and send it as response
2. The bot then reads the guild locale first to determine if its a non english locale if so it will translate the voice message to that locale and send it as response
3. It will default to english if nothing is different

### Help

If you need help using Speech To Text Bot, please use the `/help` command.

### Invite

To add Speech To Text Bot to your Discord server, please use the following invite link:

[Invite Link][invite]

### Speed

The bot's performance is currently limited due to being hosted on a suboptimal VPS, which may result in delays of up to a minute for simple text. To help improve the bot's hosting and development, please consider supporting us by sponsoring on Github at the following link: <https://github.com/sponsors/Tricked-dev>. Thank you for your understanding and support!

### Development

See [Contributing](./CONTRIBUTING.md)

### Privacy

Speech To Text Bot is committed to protecting the privacy of its users. The bot only saves the voice messages / videos for as long as it takes to translate so when the bot has responded you can be sure your attachment is deleted.

### Terms of Service

By using Speech To Text Bot, you agree to be bound by our terms of service. Please read them carefully before using the bot.

## Advanced

The bot supports any type of video/audio while transcribing

## Credits

Speech To Text Bot was created by tricked#3777. The bot uses whisper.cpp for voice transcription.

Thank you for using Speech To Text Bot!

[invite]: https://discord.com/oauth2/authorize?client_id=838065007971139594&scope=bot%20applications.commands&permissions=0
