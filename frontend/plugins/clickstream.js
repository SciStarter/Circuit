import axios from 'axios';

document.body.addEventListener('click', function(event) {
    let params = {
        session: window.localStorage.getItem("token") || "",
        on_page: window.location.pathname + window.location.search + window.location.hash,
        element_type: event.target.nodeName,
        element_id: event.target.id,
        element_class: event.target.className
    };

    const context = event.target.closest('[data-context]');
    if(!!context) {
        params.context = context.dataset.context;
    }

    switch(event.target.nodeName) {
    case 'A':
        params.text = event.target.innerText.slice(0, 32).replace('\n', ' ');
        params.href = event.target.href;
        break;
    case 'INPUT':
    case 'BUTTON':
        params.name = event.target.name;
        params.value = event.target.value;
        break;
    case 'DIV':
    case 'SPAN':
    case 'P':
        params.text = event.target.innerText.slice(0, 32).replace('\n', ' ');
        break;
    case 'IMG':
        params.text = event.target.alt;
    default:
    };

    axios.post(
        window.location.protocol + "//" + window.location.hostname + "/api/ui/click",
        params,
    );
});
