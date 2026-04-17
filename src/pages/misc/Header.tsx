import { Link } from "react-router";

const Header = () => {
    return (
        <div className="bg-primary sticky top-0 z-50 flex items-center justify-between p-4">
            <h1 className="text-4xl font-bold text-white">
                <Link to="/">UniCRM</Link>
            </h1>
        </div>
    );
};

export default Header;
