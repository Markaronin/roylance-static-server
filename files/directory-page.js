const createNewFolderButton = document.getElementById('createNewFolderButton');
const newFolderNameInput = document.getElementById('newFolderName');

function addNewSubfolderListEntry(url) {

}

createNewFolderButton.onclick = () => {
    createNewFolderButton.disabled = true;
    // const 
    fetch('/admin/create-new-directory', {
        "method": "PUT", 
        "body": JSON.stringify({
            "name": newFolderNameInput.value,
            "current_directory": window.location.pathname,
        }),
    })
    .then((response) => {
        createNewFolderButton.disabled = false;
        if (response.ok) {
            newFolderNameInput.value = "";
            addNewSubfolderListEntry(`${window.location.pathname}/${}`)
            alert("hi")
        } else {
            alert("Error while adding directory")
        }
    })
}