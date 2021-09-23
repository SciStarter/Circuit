import unzip from "lodash/unzip";

export default {
  tagged(tags, tag) {
    const split = unzip(
      tags.split(",").map((t) => t.trim()).map((t) => {
        const parts = t.split(":");
        return [parts[0].trim(), parts.slice(1).map((p) => p.trim())];
      }),
    );

    const index = split[0].indexOf(tag);

    if (index < 0) {
      return false;
    }

    return split[1][index];
  },
};
