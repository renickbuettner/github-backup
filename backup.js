const { Octokit } = require("@octokit/rest");
const fs = require('fs');

const args = process.argv.slice(2);

console.log('[Auth Key] ', args[0]);
const authkey = args[0] || null;

console.log('[User Name] ', args[1]);
const username = args[1] || null;

const config = {
    timeZone: 'Europe/Berlin',
    auth: authkey,
    userAgent: 'backup by renick.io v1.0'
};

const octokit = new Octokit(config);

console.log('[Backup][0 / 3] Starting');

try {
    fs.mkdirSync('dist');
} catch (e) {}

octokit.repos.listForAuthenticatedUser({
    username,
    type: 'owner',
    per_page: 100,
    sort: 'updated',
    direction: 'desc'
})
.then(({ data, headers, status }) => {
    fs.writeFileSync('dist/repos.json', JSON.stringify(data));
    console.log('[Backup][1 / 3] Received repos');
    const repos = [];

    data.map((repo) => {
        repos.push({
            url: repo['ssh_url'],
            changed: (repo['updated_at'].split('T'))[0],
            name: repo['full_name'].replace('/', '_')
        })
    });

    backupRepos(repos);
});

async function backupRepos(repos) {
    console.log('[Backup][2 / 3] Backup ' + repos.length + ' repos');

    for (const r of repos) {
        await _backup(r.url, r.name + '_' + r.changed);
    }

    console.log('[Backup][3 / 3] Done.');
}

async function _backup(path, fileName) {
    console.table({path, fileName});
    const zipFile = 'dist/' + fileName + '.zip';

    // delete file if already exists
    if(fs.existsSync(zipFile)) {
        fs.unlinkSync(zipFile);
    }

    await execShellCommand('git clone ' + path + ' dist/' + fileName);
    await execShellCommand('zip -r ' + zipFile + ' dist/' + fileName);
    await execShellCommand('rm -rf dist/' + fileName);
}

/**
 * Executes a shell command and return it as a Promise.
 * @param cmd {string}
 * @return {Promise<string>}
 */
function execShellCommand(cmd) {
    const exec = require('child_process').exec;
    return new Promise((resolve, reject) => {
        exec(cmd, (error, stdout, stderr) => {
            if (error) {
                console.warn(error);
            }
            resolve(stdout? stdout : stderr);
        });
    });
}