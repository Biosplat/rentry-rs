// // apiClient.js in the /static directory

// class ApiClient {
//     constructor(baseURL) {
//         this.baseURL = baseURL;
//     }

//     async createPaste(content, customSlug = null, editCode = null) {
//         try {
//             const response = await fetch(`${this.baseURL}/pastes`, {
//                 method: 'POST',
//                 headers: { 'Content-Type': 'application/json' },
//                 body: JSON.stringify({ content, customSlug, editCode })
//             });
//             if (!response.ok) throw new Error('Network response was not ok');
//             return await response.json(); // Assuming the server responds with JSON
//         } catch (error) {
//             console.error("Failed to create paste:", error);
//             throw error; // Re-throw to let caller handle it
//         }
//     }

//     async editPaste(slug, editCode, content) {
//         try {
//             const response = await fetch(`${this.baseURL}/pastes/${slug}`, {
//                 method: 'PUT',
//                 headers: { 'Content-Type': 'application/json' },
//                 body: JSON.stringify({ editCode, content })
//             });
//             if (!response.ok) throw new Error('Network response was not ok');
//             return await response.json();
//         } catch (error) {
//             console.error("Failed to edit paste:", error);
//             throw error;
//         }
//     }

//     async deletePaste(slug, editCode) {
//         try {
//             const response = await fetch(`${this.baseURL}/pastes/${slug}`, {
//                 method: 'DELETE',
//                 headers: { 'Content-Type': 'application/json' },
//                 body: JSON.stringify({ editCode })
//             });
//             if (!response.ok) throw new Error('Network response was not ok');
//             return await response.json();
//         } catch (error) {
//             console.error("Failed to delete paste:", error);
//             throw error;
//         }
//     }

//     async fetchPaste(slug) {
//         try {
//             const response = await fetch(`${this.baseURL}/pastes/${slug}`, {
//                 method: 'GET',
//             });
//             if (!response.ok) throw new Error('Network response was not ok');
//             return await response.json();
//         } catch (error) {
//             console.error("Failed to fetch paste:", error);
//             throw error;
//         }
//     }
// }

// const apiClient = new ApiClient('http://localhost:3000/api');


// apiClient.js in the /static directory

class ApiClient {
    constructor(baseURL) {
        this.baseURL = baseURL;
    }

    async createPaste(content, customSlug = null, editCode = null) {
        try {
            const response = await fetch(`${this.baseURL}/pastes`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    content: content, 
                    custom_slug: customSlug, // Adjusted to snake_case
                    edit_code: editCode // Adjusted to snake_case
                })
            });
            if (!response.ok) throw new Error('Network response was not ok');
            return await response.json(); // Assuming the server responds with JSON
        } catch (error) {
            console.error("Failed to create paste:", error);
            throw error; // Re-throw to let caller handle it
        }
    }

    async editPaste(slug, editCode, content) {
        try {
            const response = await fetch(`${this.baseURL}/pastes/${slug}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    edit_code: editCode, // Adjusted to snake_case
                    content: content
                })
            });
            if (!response.ok) throw new Error('Network response was not ok');
            return await response.json();
        } catch (error) {
            console.error("Failed to edit paste:", error);
            throw error;
        }
    }

    async deletePaste(slug, editCode) {
        try {
            const response = await fetch(`${this.baseURL}/pastes/${slug}`, {
                method: 'DELETE',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    edit_code: editCode // Adjusted to snake_case
                })
            });
            if (!response.ok) throw new Error('Network response was not ok');
            return await response.json();
        } catch (error) {
            console.error("Failed to delete paste:", error);
            throw error;
        }
    }

    async fetchPaste(slug) {
        try {
            const response = await fetch(`${this.baseURL}/pastes/${slug}`, {
                method: 'GET',
            });
            if (!response.ok) throw new Error('Network response was not ok');
            return await response.json();
        } catch (error) {
            console.error("Failed to fetch paste:", error);
            throw error;
        }
    }
}

// Assuming you have an instance of ApiClient available in your scripts
// You can create an instance like so:
const apiClient = new ApiClient('http://localhost:3000/api');