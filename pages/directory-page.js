const createNewFolderButton = document.getElementById('createNewFolderButton');
const newFolderNameInput = document.getElementById('newFolderName');
const subfileList = document.getElementById('subfileList');
const newFileInput = document.getElementById('newFile');
const newFileNameInput = document.getElementById('newFileName');
const uploadFileButton = document.getElementById('uploadFileButton');

function renderLinks() {
    function getLink(subpath) {
        const listElement = document.createElement('li');

        const link = document.createElement('a');
        link.setAttribute('href', getSubFileName(subpath));
        link.innerText = subpath;
        listElement.appendChild(link);

        const spacerSpan = document.createElement('span');
        spacerSpan.innerText = " - ";
        listElement.appendChild(spacerSpan);

        const deleteButton = document.createElement('button');
        deleteButton.innerText = "Delete";
        deleteButton.onclick = () => deleteFile(subpath);
        listElement.appendChild(deleteButton);

        return listElement;
    }
    const links = data.sort().map(getLink);
    subfileList.innerHTML = "";
    subfileList.append(...links)
}

function getSubFileName(filename) {
    return `${window.location.pathname}/${filename}`.replace("//", "/")
}

function addNewSubfolderListEntry(subfileName) {
    data.push(subfileName);
    renderLinks();
}

function deleteFile(subfileName) {
    if (confirm(`Are you sure you want to delete ${subfileName}`)) {
        fetch(getSubFileName(subfileName), {
            "method": "DELETE",
        })
        .then((response) => {
            if (!response.ok) {
                alert("Error while deleting file")
            }
        })
        data = data.filter(val => val !== subfileName);
        renderLinks();
    }
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

renderLinks();