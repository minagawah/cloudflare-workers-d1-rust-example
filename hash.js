const CHARS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
const CHARSIZE = CHARS.length;
const SIZE = 64;

for (let i = 0; i < 3; i++) {
  const date = new Date().toUTCString().replace(' GMT', '');
  let hash = '';
  let i = 0;
  while (i < SIZE) {
    hash += CHARS.charAt(Math.floor(Math.random() * CHARSIZE));
    i++;
  }
  console.log('(');
  console.log(`  '${hash}',`);
  console.log(`  '${date} +0000',`);
  console.log(')');
}
