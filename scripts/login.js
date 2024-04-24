var container = document.getElementById("container");

var buttonText = document.getElementById("proceed");
var toggleText = document.getElementById("modeToggle");

function toggleMode() {
    container.ariaLabel = container.ariaLabel == "login" ? "sign-up" : "login";

    buttonText.innerHTML = container.ariaLabel == "login" ? "login" : "sign up";
    toggleText.innerHTML = container.ariaLabel == "login" ? "dont have an account? sign up" : "have an account? login";
}

toggleMode();

document.querySelectorAll("#container input").forEach((element) => {
    element.addEventListener("keypress", (event) => {
        if (event.key == "Enter") {
            event.preventDefault();
            confirm();
        }
    })
});

async function confirm() {
    var username = document.getElementById("username").value;
    var password = document.getElementById("password").value;
    var confirm_password = document.getElementById("confirm-password").value;

    var result = document.getElementById("error");

    if ((!username) || (!password)) {
        result.innerHTML = "fill in all required data";
        return;
    }

    if ((container.ariaLabel != "login") && (password != confirm_password)) {
        result.innerHTML = "passwords do not match";
        return;
    }

    sendPostRequest(`${BACKEND_ADDRESS}/` + (container.ariaLabel == "login" ? "login" : "sign_up"), JSON.stringify({
        "username": username,
        "password": password
    }), (r) => {
        var response = JSON.parse(r);

        // console.log(response);

        if (response.type == "success") {
            setLocalStorage("chronos_username", username);
            setLocalStorage("chronos_password", password);

            // let now = new Date();
            // (now.setTime(now.getTime() + (14 * 86400000)));
            // document.cookie = `chronos_user_id=${response.user_id}; expires=${now.toUTCString()}; path=/`;

            window.location.href = "./index.html";
        }

        result.innerHTML = {
            "UsernameNoExist": "username doesnt exist",
            "PasswordWrong": "password is incorrect",
            "UserIDNoExist": "username doesnt exist",
            "success": "",

            "UsernameTaken": "username is already taken"
        }[response.type];
    })
}
