import http from 'k6/http';

export const options = {
	discardResponseBodies: true,
	scenarios: {
		contacts: {
			executor: 'constant-vus',
			vus: 10,
			duration: '30s',
		},
	},
};

export default function () {
	const response = http.get('http://localhost:8080');
 }
