// SPDX-License-Identifier: MIT

pragma solidity ^0.8.21;

interface IERC721Receiver {

    function onERC721Received(
        address operator,
        address from,
        uint256 tokenId
    ) external returns (bytes4);
}


contract ERC721ReceiverMock is IERC721Receiver {
    enum RevertType {
        None,
        RevertWithoutMessage,
        RevertWithMessage,
        RevertWithCustomError,
        Panic
    }

    bytes4 private immutable _retval;
    RevertType private immutable _error;

    event Received(address operator, address from, uint256 tokenId);

    error CustomError(bytes4);

    constructor(bytes4 retval, RevertType error) {
        _retval = retval;
        _error = error;
    }

    function onERC721Received(
        address operator,
        address from,
        uint256 tokenId
    ) public returns (bytes4) {
        if (_error == RevertType.RevertWithoutMessage) {
            revert();
        } else if (_error == RevertType.RevertWithMessage) {
            revert("ERC721ReceiverMock: reverting");
        } else if (_error == RevertType.RevertWithCustomError) {
            revert CustomError(_retval);
        } else if (_error == RevertType.Panic) {
            uint256 a = uint256(0) / uint256(0);
            a;
        }

        emit Received(operator, from, tokenId);
        return _retval;
    }
}
