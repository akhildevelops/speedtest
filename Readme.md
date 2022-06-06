# Speed Test your Internet  🚀

Speed test internet connection and display them in a graph daily using Airtable.

![Network-Speed](https://i.imgur.com/LevGVsb.png)

This App uses speedtest-cli provided by [speedtest.net](https://www.speedtest.net/)

# Setup
- [Install](https://www.speedtest.net/apps/cli) speedtest cli App.
- Run `speedtest` command from terminal and accept T&C. After a while we get speedtest results in the terminal.
![speedtes-reulst](https://i.imgur.com/mXMnnAY.png)
- [Setup Airtable](https://airtable.com/signup) account and create a table.
- Generate API KEY from [airtable-account](https://airtable.com/account).
- Click on a base from this link: [Airtable Base ID](https://airtable.com/api), to fetch base id.
- Create environment variables with the values fetched from above:
```bash
AIRTABLE_BASE_ID=xxxxxxx
AIRTABLE_AUTH_KEY=xxxxxx
```
- git clone the repo and `cargo run` from the root of the repository.
- After verifying the speedtest results in Airtable. [Add Vega-lite](https://i.imgur.com/RvTOkLq.mp4) App from applications section to visuakize the graph. [Refer here](https://vega.github.io/vega-lite/examples/line.html) for a line graph example.

