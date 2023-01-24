export default ({ app }, inject) => {
    function render_tsv_item(item) {
        if(item === undefined || item === null) {
            return "";
        }

        return String(item)
            .replace(/\\/g, '\\\\')
            .replace(/\r/g, '\\r')
            .replace(/\n/g, '\\n')
            .replace(/\t/g, '\\t');
    }

    function render_csv_item(item) {
        if(item === undefined || item === null) {
            return "";
        }

        item = String(item);

        if(item.indexOf("\r") > -1 || item.indexOf("\n") > -1 || item.indexOf("\"") > -1 || item.indexOf(",") > -1) {
            return `"${item.replace(/"/g, '""')}"`;
        }

        return item;
    }

    function render_delimited_row(delimiter, row_terminator, render_item, columns, row) {
        let out = [];

        for(let col of columns) {
            out.push(render_item(row[col]));
        }

        return out.join(delimiter) + row_terminator;
    }

    function render_delimited_header(delimiter, row_terminator, render_item, columns) {
        let out = [];

        for(let col of columns) {
            out.push(render_item(col));
        }

        return out.join(delimiter) + row_terminator;
    }

    function save_blob(filename, blob) {
      const link = document.createElement("a");
      link.href = URL.createObjectURL(blob);
      link.download = filename;
      link.click();
      URL.revokeObjectURL(link.href);
    }

    function save_table_csv(base_filename, columns, array) {
        let chunks = [];

        chunks.push(render_delimited_header(",", "\r\n", render_csv_item, columns));

        for(let row of array) {
            chunks.push(render_delimited_row(",", "\r\n", render_csv_item, columns, row));
        }

        const blob = new Blob(chunks, { type: "text/csv" });
        save_blob(base_filename + '.csv', blob);
    }

    function save_table_tsv(base_filename, columns, array) {
        let chunks = [];

        chunks.push(render_delimited_header("\t", "\r\n", render_tsv_item, columns));

        for(let row of array) {
            chunks.push(render_delimited_row("\t", "\r\n", render_tsv_item, columns, row));
        }

        const blob = new Blob(chunks, { type: "text/tab-separated-values" });
        save_blob(base_filename + '.tsv', blob);
    }

    inject('save_blob', save_blob);
    inject('save_table_csv', save_table_csv);
    inject('save_table_tsv', save_table_tsv);
}
