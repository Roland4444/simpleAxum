function showMessage() {
    const messageElement = document.getElementById('message');
    const now = new Date();
    messageElement.textContent = `Button clicked at ${now.toLocaleTimeString()}!`;
    
    // Fetch data from /test endpoint
    fetch('/test')
        .then(response => response.text())
        .then(data => {
            console.log('Response from /test:', data);
        });
}

console.log('index.js loaded successfully');