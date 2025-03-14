async function fetchCryptoData() {
    const response = await fetch('/api/crypto');
    const data = await response.json();
    const cryptoList = document.getElementById('crypto-list');

    data.forEach(crypto => {
        const cryptoItem = document.createElement('div');
        cryptoItem.innerHTML = `
            <strong>${crypto.name} (${crypto.symbol})</strong>: $${crypto.quote.usd.price.toFixed(2)}
        `;
        cryptoList.appendChild(cryptoItem);
    });
}

fetchCryptoData();