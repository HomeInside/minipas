document.addEventListener("DOMContentLoaded", function () {
    /*document.querySelectorAll("pre code").forEach((block) => {
        hljs.highlightBlock(block);
        block.classList.add("hljs"); // Asegúrate de que el estilo sea aplicado
    });*/
	hljs.highlightAll();
    hljs.initLineNumbersOnLoad(); // Inicializa la numeración de líneas
});
