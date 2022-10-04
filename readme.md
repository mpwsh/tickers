## Description
Simple token price data exposed through an actix server.  
Data is retrieved from `OKX` exchange, but the code is so simple you can modify it to suit your needs if thats required.  
Built to display prices on my Nokia N9 using [Billboard](https://github.com/harmattan/billboard). Long live Meego Harmattan! <3


## Build
```bash
#Build
cargo build --release
```

## Start
```bash
#Run the server
./target/release/tickers
```

## Usage
```bash
#Base entrypoint
curl http://localhost:3030
#Response
======= Tickers Simple =======
Github: https://github.com/mpwsh/simple-tickers
Author: mpw <x@mpw.sh>
==============================
Available endpoints:
  Single:
  - /price/<inst_id> -- Example: /price/XCH-USDT
  Multiple:
  - /price/<inst_id>,<inst_id> -- Example: /price/SOL-USDT,XCH-USDT
```

Single token price check
```bash
#Check a single token price
curl  http://localhost:3030/price/SOL-USDT
# Response
SOL: ⬆ 3.27% 34.08 USDT
```

Multiple tokens
```bash
# Check price of multiple tokens
curl  http://localhost:3030/price/SOL-USDT,XCH-USDT,ETH-USDT,BTC-USDT,FTM-USDT
# Response
SOL: ⬆ 3.21% 34.07 USDT
XCH: ⬆ 1.63% 34.06 USDT
ETH: ⬆ 2.75% 1357.17 USDT
BTC: ⬆ 3.58% 20268.9 USDT
FTM: ⬆ 2.40% 0.2282 USDT
```
