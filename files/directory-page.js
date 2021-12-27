const createNewFolderButton = document.getElementById('createNewFolderButton');
const newFolderNameInput = document.getElementById('newFolderName');
const subfileList = document.getElementById('subfileList');
const newFileInput = document.getElementById('newFile');
const newFileNameInput = document.getElementById('newFileName');
const uploadFileButton = document.getElementById('uploadFileButton');

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
    fetch(`${window.location.pathname}/${newFolderNameInput.value}`, {
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

uploadFileButton.onclick = (event) => {
    const file = newFileInput.files[0];
    const name = newFileNameInput.value;
    if (file && name && !name.includes(" ")) {
        console.log(file);
        const formData = new FormData()
        formData.append('myFile', file)
        fetch('10.0.0.186', {
            method: 'POST',
            body: formData
        })
    } else {
        alert("File or name are invalid")
    }
}