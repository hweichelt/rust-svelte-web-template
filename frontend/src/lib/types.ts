export type User = {
	id: string;
	email: string;
	display_name: string;
	created_at: string;
};

export type ApiErrorBody = {
	code: string;
	message: string;
};
