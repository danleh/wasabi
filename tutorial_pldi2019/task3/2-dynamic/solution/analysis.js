let decrypted = [];

function getDecrypted() {
    alert(String.fromCharCode(...decrypted));
}

Wasabi.analysis = {
    store: function(location, op, addr, value) {
        console.log(arguments);
        decrypted.push(value);
    },
};
