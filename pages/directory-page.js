const createNewFolderButton = document.getElementById('createNewFolderButton');
const newFolderNameInput = document.getElementById('newFolderName');
const subfileList = document.getElementById('subfileList');
const newFileInput = document.getElementById('newFile');
const newFileNameInput = document.getElementById('newFileName');
const uploadFileButton = document.getElementById('uploadFileButton');

function getSubFileName(filename) {
    return `${window.location.pathname}/${filename}`.replace("//", "/")
}

function addNewSubfolderListEntry(subfileName) {
    let subfileLink = subfileName;
    if (window.location.pathname !== "/") {
        subfileLink = `${window.location.pathname}/${subfileName}`;
    }
    const newLi = document.createElement('li');
    const newAnchor = document.createElement('a');
    newAnchor.innerText = subfileName;
    newAnchor.setAttribute("href", subfileLink);

    newLi.appendChild(newAnchor);
    subfileList.appendChild(newLi);
}

createNewFolderButton.onclick = () => {
    createNewFolderButton.disabled = true;
    fetch(getSubFileName(newFolderNameInput.value), {
        "method": "POST",
    })
    .then((response) => {
        createNewFolderButton.disabled = false;
        if (response.ok) {
            addNewSubfolderListEntry(newFolderNameInput.value)
            newFolderNameInput.value = "";
        } else {
            alert("Error while adding directory")
        }
    })
}

uploadFileButton.onclick = () => {
    const file = newFileInput.files[0];
    const name = newFileNameInput.value;
    if (file && name && !name.includes(" ")) {
        uploadFileButton.disabled = true;
        fetch(getSubFileName(name), {
            method: 'PUT',
            body: file
        })
        .then((response) => {
            uploadFileButton.disabled = false;
            if (response.ok) {
                addNewSubfolderListEntry(name)
                newFileNameInput.value = "";
                newFileInput.value = null;
            } else {
                alert("Error while uploading file")
            }
        })
    } else {
        alert("File or name are invalid")
    }
}