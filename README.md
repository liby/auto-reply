## Auto-reply (out of office)

生成外出或无法回复电子邮件时的自动回复模板，并自动发送给 Bot。

## Usage

- Settings / Secrets / Actions / Repository secrets
    1. 设置 `TELEGRAM_CHAT_ID`，你可以使用 [@RawDataBot](https://t.me/RawDataBot) 来获取它
    2. 设置 `TELEGRAM_TOKEN`，Telegram bot token
    3. 设置 `HOLIDAYS`，格式为 `2020-01-01,2020-01-02,2020-01-03`（开始日期，结束日期，上班日期），上班日期可省略，默认为结束日期后一天。
- Run workflow

## Templates

- 短期节假日（1-3天）

    > I will be out of the office from Thursday, January 23, 2020 to Friday, January 24, 2020. I will respond to your email when I return to work on Monday.
    >

- 长期节假日（一周或更长）

    > I will be out of office from Thursday, January 23, 2020 through Wednesday, January 29, 2020. I will be back to office on Thursday, January 30.
    >
    > During this period I will have limited access to my email. I will try to respond to your email as soon as I can.
    >