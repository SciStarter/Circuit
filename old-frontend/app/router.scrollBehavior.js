// Credit to Zach Cardoza for suggesting this hash scrolling approach
export default async function (to, _, savedPosition) {
  if (savedPosition) {
    return savedPosition;
  }

  function find(hash, resolve) {
    if (resolve === undefined) {
      return new Promise((resolve) => {
        find(hash, resolve);
      });
    }

    const found = document.querySelector(hash);

    if (found !== null) {
      return resolve(found);
    } else {
      setTimeout(() => {
        find(hash, resolve);
      }, 100);
    }
  }

  if (to.hash) {
    const el = await find(to.hash);
    return { x: 0, y: el.offsetTop - 100 };
  }

  return { x: 0, y: 0 };
}
