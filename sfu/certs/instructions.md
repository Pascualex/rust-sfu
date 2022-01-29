Steps extracted from Google Chrome's example server:
https://github.com/GoogleChrome/samples/blob/gh-pages/webtransport/webtransport_server.py

Create the DER certificate and the key:
openssl req -newkey rsa:2048 -nodes -keyout server.pem -x509 -out server.crt -outform DER -subj '/CN=Test Certificate' -addext "subjectAltName = DNS:localhost"

Encode the key to DER format:
openssl rsa -in server.pem -out server.key -outform DER

Obtain the fingerprint of the certificate (MIGHT FAIL IF DONE IN WINDOWS):
openssl x509 -pubkey -noout -in server.crt -inform DER | openssl rsa -pubin -outform der | openssl dgst -sha256 -binary | openssl base64 -e

Run chrome with the following flags:
.\chrome.lnk --origin-to-force-quic-on=$URL --ignore-certificate-errors-spki-list=$FINGERPRINT
