# coinbase-connector

SSL
sudo apt install libssl-dev
git clone git://git.openssl.org/openssl.git
./config --openssldir=/usr/local/ssl
sudo make install
sudo apt install pkg-config

docker build -t coinbase-connector .
docker run -it --network host --rm coinbase-connector

export JWT=
curl -H "Authorization: Bearer $JWT" 'https://api.coinbase.com/api/v3/brokerage/accounts'
